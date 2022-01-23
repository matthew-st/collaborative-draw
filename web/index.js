const WEBSOCKET_URL = "wss://ws-draw.ocld.cc"
const RECAPTCHA_SITE_KEY = "6LfvGjEeAAAAAIkQH7kmoV7mJCgASq3fIYsi_cif"

class Drawing {
    constructor(socket_url, recaptcha_token) {
        this.socket = new WebSocket(socket_url)
        this.socket.onopen = () => {
            this.socket.send(JSON.stringify({
                "captcha": recaptcha_token
            }))
        }
        this.socket.onmessage = (event) => {
            let object = JSON.parse(event.data.toString())
            switch (object.t) {
                case "init":
                    this.init(object.d)
                case "update":
                    this.update(object.d)
            }
        }
    }
}

function reCAPTCHAsubmit(e) {
    e.preventDefault()
    grecaptcha.ready(function() {
        grecaptcha.execute(RECAPTCHA_SITE_KEY, {action: 'submit'}).then(function(token) {
            new Drawing(WEBSOCKET_URL, token)
        })
    })
}