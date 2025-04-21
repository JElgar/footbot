# Footbot

Sends whatsapp message every week to get sign ups for next week and remind people to pay

## Build and deploy

```
cd footbot_service && cargo lambda build --release --arm64 --output-format zip
cd footbot_infra && npm run deploy
```
