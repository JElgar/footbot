# Footbot

Sends whatsapp message every week to get sign ups for next week and remind people to pay

## Build and deploy

### Build service

```
cd footbot_service
export WAPI_TOKEN=<YOUR_TOKEN>
cargo lambda build --release --arm64 --output-format zip
```

```
cd footbot_infra
export AWS_ACCESS_KEY_ID=<>
export AWS_SECRET_ACCESS_KEY=<>
npm run deploy
```
