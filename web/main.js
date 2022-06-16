const data = require("./good.json");
const keys = new Set(Object.keys(data));
const has = x => keys.has(x.toISOString().split('T')[0])

console.log("Dates not found:")

let date = new Date("1900-01-01");
while (date.getFullYear() < 2100) {
    date.setDate(date.getDate()+1);
    if (!has(date)) console.log(date);
}

console.log("Best dates")
const sorted = Object.entries(data).sort((a,b) => b[1]-a[1]);
for (let i=0; i<10; i++) {
    console.log(sorted[i]);
}