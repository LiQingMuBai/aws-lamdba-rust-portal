# aws-lamdba-rust-portal
![categorize](https://res.cloudinary.com/lumigo-production/fl_lossy,f_auto/wp-website/2019/06/APIGW-Lambda-1024x593.png)

## Building the crate in OSX

In order to build this project in OSX you must ensure that you have a folder called `.cargo` with a `config` file in it. This file must contain exactly this:

```sh
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```
## Building the crate in Linux

Install the `musl-tools`:

```sh
sudo apt install musl-tools
```

Remove the `.cargo` folder if present or comment the contents of the `config` file.

## Common building steps

```sh
# this will start the build process
cargo build --release --target x86_64-unknown-linux-musl
# this will create the final zip
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```

## A BRIEF OVERVIEW OF API GATEWAY
![categorize](https://res.cloudinary.com/lumigo-production/fl_lossy,f_auto/wp-website/2019/06/API-Gateway-flow-Amazon.jpg)


## AWS RUST LAMDBA 中文说明
[说明](https://amazonaws-china.com/cn/blogs/china/rust-runtime-for-aws-lambda/)

### AWS Setup
Several AWS resources are needed to follow along in the content. This [guide](https://github.com/LiQingMuBai/aws-lamdba-rust-portal/blob/master/AWS_SETUP.md)
will help you set up the resources needed and will show you how to remove everything
as well.

