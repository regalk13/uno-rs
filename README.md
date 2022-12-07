<br />
<div align="center">
  <img src="https://user-images.githubusercontent.com/72028266/205465535-beeedf0b-24fb-4c88-9c53-c8312c332b55.png" alt="Logo" width="140" height="100">
  

  <h3 align="center">Uno rust</h3>

  <p align="center">
    The famous Uno card game made in Rust programming language!
    <br>
    Using <a href="https://yew.rs/">Yew</a>(Frontend) + <a href="https://actix.rs/">Actix</a>(Backend) frameworks for running it on a browser.   

  </p>
</div>



> Create a room share it to your friends and have a good time! 

This project is under a lot of work and is still in an early stage.
## Why rust?

As you can see this project is written in Rust, both in frontend using [web assembly](https://webassembly.org/), and in backend. Which for web apps may be different from the standard but there are good reasons for its use. The first one is performance, although WASM is still developing many interactions that still need JS underneath, which makes it not fast enough even so it's pretty *blazingly fast* using SSR and compression for small space payloads. Another reason and the most important is the functionalities that the language offers you, which allows us to make the implementation very comfortable in development, without to mention another multiple benefits that it brings.

## todo!()

### UI

- [ ] Home page view
- [x] Navbar component
- [ ] Room view
- [ ] Profile view

### Backend

- [ ] Auth
- [ ] Apply JWT and cookies
- [ ] Implement WebSockets
- [ ] Create DataBase (SQL)
    - [ ] Add DB models
    - [ ] Add users DB
### API

- [ ] Create API
- [ ] ...

### DevOps

- [ ] Add Docker
