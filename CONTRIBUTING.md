# Contributing to Thetawave

## Building and Running Tests

```bash
cargo test --workspace --all-features
```

Running the game locally
```
cargo run --release --features storage
```

Building the game with arcade functionality.

```
cargo build --release features storage --features arcade
```

Build and run the game for wasm. 

Run 
```bash
./build_wasm.sh
```
Then serve the `out/` directory with some HTTP server.

For example

```bash
python3 -m http.server -d out/
```

## Assets

We choose to store most images and audio (10Mb-1Gb) in [AWS S3](https://aws.amazon.com/s3/), rather than in git.


To contribute to the assets, do the following.

0. Create an AWS account.
0. [Create an IAM User](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_users_create.html) on your AWS account. This
   is distinct from your account's root user. Let's call this IAM user `mythetawavedev`. 
0. Send a repo maintainer your IAM User ARN. It will look something like
   `arn:aws:iam::121767489999:user/mythetawavedev`. The maintainer will allow you to use an AWS IAM Role that gives
short term access to the assets 
0. For web browser/console/interactive access [see the ThetawaveDeveloperRole IAM role login](
   https://signin.aws.amazon.com/switchrole?roleName=ThetawaveDeveloperRole&account=6564541241021).  For example, in the
[`assets-thetawave` bucket console](https://s3.console.aws.amazon.com/s3/buckets/assets-thetawave?) one can manually
upload assets.
0. To download all of the assets from a local development environment using the
   [aws-cli](https://github.com/aws/aws-cli), I recommend creating an `~/.aws/credentials` file like the following.
   ```ini
   [thetawavedev]
   aws_access_key_id = <MY_ACCESS_KEY>
   aws_secret_access_key = <MY_SECRET_KEY>
   region = us-east-2
   ```

   and an ~/.aws/config` file  like

   ```ini
   [profile thetawavedev-p]
   role_arn = arn:aws:iam::656454124102:role/ThetawaveDeveloperRole
   source_profile = thetawavedev
   role_session_name = <A_NAME_FOR_YOUR_SESSION>
   ```
   Then run `./asset_manager.py download free_assets` or `./asset_manger.py --profile <YOUR_AWS_PROFILE_NAME> download
   premium_assets` to invoke the AWS CLI. Or look at that script to figure out how to invoke the AWS CLI yourself. AWS
   has more docs about [configuring
   credentials]((https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)
0. If you are using another open source tool for interacting with S3, such as
   [`rclone`](https://github.com/rclone/rclone), you may need to use `--s3-profile` or do something like the following.
```bash
export AWS_SESSION_TOKEN=$(aws sts assume-role --profile thetawavedev --output text \
    --role-arn arn:aws:iam::656454124102:role/ThetawaveDeveloperRole \
    --role-session-name vpsession --query="Credentials.SessionToken" )
```
