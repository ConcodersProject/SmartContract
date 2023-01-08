# deploying 

```bash
concordium-client module deploy dist/module.wasm.v1 --sender weeblet --name market-test --grpc-port 10001 --grpc-ip shebuilds.concordium.com
```
# Contract init

```bash
concordium-client --grpc-ip shebuilds.concordium.com contract init 137fd3cdebe80b5f1739bc45066034791e6019f169470709292dc948e046e001 --sender weeblet --energy 10000 --contract market
```
