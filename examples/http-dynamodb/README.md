# AWS Lambda Function example

## Build & Deploy

1. Install [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda#installation)
2. Build the function with `cargo lambda build --release` 
3. Deploy the function to AWS Lambda with `cargo lambda deploy --iam-role YOUR_ROLE`

## Build for ARM 64

Build the function with `cargo lambda build --release --arm64` 

Setting up Dynamodb

1. Log into your account.
2. Create a Dynamodb table with the name 'lambda_dyno_example' with the partition key of "username".
3. Create IAM role with the permissions for Lambda, Cloudwatch and Dynamodb.


