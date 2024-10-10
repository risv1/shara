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

app.listen(8000, () => {
    console.log("Server is running on port 8000")
})