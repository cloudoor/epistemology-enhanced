<p align="center"><img width="300" alt="Screenshot 2024-01-15 at 1 14 11 PM" src="https://github.com/cloudoor/epistemology-enhanced/assets/294042/4e4f6e66-52b0-43b1-87df-8a7c86e3b5c0"></p>

A simple way to run a [llama.cpp](https://github.com/ggerganov/llama.cpp) executable via a local private HTTP API for completions and embeddings.

Privacy goals:
* server is stateless
* always run on localhost only
* never write logs
* never put prompts in console logs
* **MIT** license so you can modify this to your specific needs at whim

The goal of this project is to make a completely clear and visible way to run a server locally. The code for how this runs is as minimal as possible so you can understand exactly what you are running.

You can install by grabbing binaries for all operating systems from [Releases](https://github.com/cloudo