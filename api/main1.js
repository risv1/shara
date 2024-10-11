import express from "express"
import cors from "cors"

const app = express()

app.use(cors())

app.get("/", (req, res) => {
    res.send("Hello World 0")
})

app.get("/api", (req, res) => {
    res.send("API 0")
})

const delay = ms => new Promise(resolve => setTimeout(resolve, ms))

app.get("/api/test", async (req, res) => {
    await delay(1000)
    res.send({
        message: "Test response",
        timestamp: new Date().toISOString()
    })
})

app.listen(8001, () => {
    console.log("Server is running on port 8001")
})