using System.Globalization;
using System.Text.Json;

// using FileStream fs = File.Open("../resources/pi-billion.txt", FileMode.Open, FileAccess.Read, FileShare.Read);
using FileStream fs = File.Open("C:/Users/TeamD/Downloads/pi-billion.txt", FileMode.Open, FileAccess.Read, FileShare.Read);
using BufferedStream bs = new BufferedStream(fs);
using StreamReader sr = new StreamReader(bs);


Dictionary<string, List<int>> count = new();

var executing = true;
var cancelled = false; // only trap ctrl+c once
void onCancel(object? sender, ConsoleCancelEventArgs e)
{
    if (cancelled) 
    {
        Console.WriteLine("Exiting fr");
        return;
    }
    cancelled = true;

    e.Cancel = true;
    Console.WriteLine("Detected early exit");
    executing = false;
}


Console.TreatControlCAsInput = false;
Console.CancelKeyPress += onCancel;


string current = "........";

int read;
char[] buffer = new char[10 * 1024 * 1024];
long numChunks = fs.Length / buffer.Length;
Console.WriteLine("Reading, length {0}, {1} chunks", fs.Length, numChunks);
int i=0;
int j=0;
while (executing && (read = sr.ReadBlock(buffer)) > 0)
{
    foreach (char c in buffer)
    {
        if (!executing) break;
        j++;
        current = current.Substring(1) + c;
        if (current.StartsWith("19") || current.StartsWith("20")) 
        {
            List<int>? old;
            if (!count.TryGetValue(current, out old))
            {
                old = new();
                count[current] = old;
            }
            old.Add(j-7);
        }
    }
    Console.WriteLine("Finished chunk {0} of {1} ({2}%)", i, numChunks, i*100/numChunks);
    i++;
}

Console.WriteLine("Processing data, found {0} entries", count.Count);
Dictionary<string, List<int>> good = new();
foreach (var x in count)
{
    if (!x.Key.StartsWith("19") && !x.Key.StartsWith("20")) continue;
    try
    {
        var date = DateTime.ParseExact(x.Key, "yyyyMMdd", CultureInfo.InvariantCulture);
        var display = date.ToString("yyyy-MM-dd");
        good[display] = x.Value;
    }
    catch { }
}
Console.WriteLine("Writing data");
// File.WriteAllText("./out.txt", JsonSerializer.Serialize(count, new JsonSerializerOptions { WriteIndented = true, }));
File.WriteAllText("./good.txt", JsonSerializer.Serialize(good, new JsonSerializerOptions { WriteIndented = true, }));