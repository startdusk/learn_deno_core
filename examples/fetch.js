function print(data) {
    Deno.core.print(`${data}\n`)
}

print("starting to fetch...")
let res = await fetch({ url: "http://www.baidu.com" })
print(`status: ${res.status}`)
print(`headers: ${JSON.stringify(res.headers, null, 2)}`)
print(`text: ${res.text()}`)
