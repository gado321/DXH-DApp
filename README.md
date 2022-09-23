DXH DApp
==================

## How to run:

First, deploy DXH_Token and save the contract ID. Then, you should change the TOKEN_SC_ADDR in `contract/src/lib.rs` and contractId in donate function of `frontend/near-interface.js`

```
yarn install
cd frontend && yarn install
cd ..
yarn deploy
yarn start
```

## Documents

[Slides](./documents/DXH_Slides.pdf)

[White_paper](./documents/Bản%20tóm%20tắt%20nền%20tảng%20từ%20thiện%20phi%20tập%20trung%20DXH.pdf)

[Demo_video](https://youtu.be/oOmi1_Y0yG8)