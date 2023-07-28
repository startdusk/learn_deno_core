import { print } from "./base.js"
try {
    const res = await fetch("https://jsonplaceholder.typicode.com/todos/1")
    const json = await res.json()
    print(json)
} catch (error) {
    print(error)
}
