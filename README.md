# Plotly in Wasm Environment
This is a simple repo to demonstrate how to use Plotly in a WebAssembly environment. If everything works, it should be integrated in the [Earth Metabolome Initiative (EMI)](https://github.com/earth-metabolome-initiative/emi-monorepo) project.


## How to run
You should have trunk installed. 
```bash
 cargo install --locked trunk
```

Then you can run the following command to start the server.
```bash
trunk serve --open
```

This should open a new tab in your browser with the Plotly example.