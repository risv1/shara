import express from "express"
import cors from "cors"

const app = express()

app.use(cors())

app.get("/", (req, res) => {
    res.send("Hello World 1")
})

app.get("/api", (req, res) => {
    res.send("API 1")
})

app.listen(8001, () => {
    console.log("Server is running on port 8001")
})