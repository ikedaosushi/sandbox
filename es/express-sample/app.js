const express = require('express');
const cors = require('cors');
const axios = require('axios');
const bodyParser = require('body-parser');
const querystring = require('querystring');

const app = express()
app.use(cors({ origin: true }));
app.use(bodyParser.urlencoded({extended: true}));

app.get('/', (req, res) => res.send('Hello World!'))

app.all("/spotify", (req, resp) => {
    const baseUrl = "https://api.spotify.com"
    const path = req.query.path
    const url = baseUrl + path
    let headers = {
        "Authorization": req.get('Authorization')
    }

    axios({
        method: req.method,
        url: url,
        headers: headers
    }).then(r => {
        resp.status(r.status).json(r.data)
    }).catch(e => {
        resp.status(e.response.status).json(e.response.data)
    })
})

app.all("/spotify_account", (req, resp) => {
    const baseUrl = "https://accounts.spotify.com"
    const path = req.query.path
    const url = baseUrl + path
    let headers = {
        "Authorization": req.get('Authorization')
    }
    const data = querystring.stringify(req.query)

    axios({
        method: "POST",
        url: url,
        data: data,
        headers: headers
    }).then(r => {
        resp.status(r.status).json(r.data)
    }).catch(e => {
        resp.status(e.response.status).json(e.response.data)
    })
})



const port = 7000
app.listen(port, () => console.log(`Example app listening on port ${port}!`))