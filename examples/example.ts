

interface Foo {
    bar: string;
    fizz: number;
}

console.log("Hello", "runjs!");

let content = await fetch("https://deno.land/std@0.177.0/examples/welcome.ts");
console.log("Content from fetch", content);

