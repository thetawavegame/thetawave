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

We choose to store most images and audio (10Mb-1Gb) in [AWS S3](https://aws.amazon.com/s3/), rather than in git for the
following reasons:
- Git is good for determining text-based diffs and has reasonable merge strategies for text, but is not so useful for
  binary files like audio and images.
- Time consuming `git clone` runs are annoying.
- We may segregate "free" vs "premium" assets.

Some day we may try [git annex](https://git-annex.branchable.com/), [git LFS](https://github.com/git-lfs/git-lfs), or
some more dedicated service that offers peer-review and s3-like operations of a distributed, versioned filesystem.

To contribute to the assets, do the following.

0. [Create an AWS account](https://docs.aws.amazon.com/accounts/latest/reference/manage-acct-creating.html).
0. [Create an IAM User](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_users_create.html) on your AWS account. This
   is distinct from your account's root user, and will only have permissions to contribute to Thetawave. Let's call this
   IAM user `mythetawavedev`. While slightly more complex than 1 user account/username+password pair, keeping deliberate
   and minimal permissions is a [standard infosec](https://en.wikipedia.org/wiki/Principle_of_least_privilege) and [AWS
   IAM best practice](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html#grant-least-privilege).
0. Send a repo maintainer your IAM User ARN. It will look something like
   `arn:aws:iam::121767489999:user/mythetawavedev`. The maintainer will allow you to use an AWS IAM Role that gives
   short term access to the assets.
0. Using your root account (or one with privileges to manipulate IAM permissions), give your thetawave-specific IAM user
   permission to access our IAM role. [Create](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_create-console.html) and
   [attach](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-create-and-attach-iam-policy.html)
   the following policy to your (nonroot) user.

   ```json
   {
     "Version": "2012-10-17",
     "Statement": [
       {
         "Effect": "Allow",
         "Action": [
           "sts:AssumeRole"
         ],
         "Resource": [
           "arn:aws:iam::656454124102:role/ThetawaveDeveloperRole"
         ]
       }
     ]
   }
   ```
   [See AWS docs for more instructions](https://repost.aws/knowledge-center/cross-account-access-iam).

0. *(Optional)* For web browser/console/interactive access, log in as your IAM user account (not the root account) and [see the
   ThetawaveDeveloperRole IAM role login](
   https://signin.aws.amazon.com/switchrole?roleName=ThetawaveDeveloperRole&account=656454124102). For example, in the
   [`assets-thetawave` bucket console](https://s3.console.aws.amazon.com/s3/buckets/assets-thetawave?) one can manually
   upload assets. This is useful for making sure permissions are set up properly, but is unnecessary for day-to-day
   operations.
0. To download all of the assets from a local development environment using the
   [aws-cli](https://github.com/aws/aws-cli), I recommend putting [long term access
   keys](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html) in an `~/.aws/credentials`
   file like the following
   ```ini
   [thetawavedev]
   aws_access_key_id = <MY_ACCESS_KEY>
   aws_secret_access_key = <MY_SECRET_KEY>
   region = us-east-2
   ```

   and manually creating an ~/.aws/config` file like

   ```ini
   [profile thetawavedev-p]
   role_arn = arn:aws:iam::656454124102:role/ThetawaveDeveloperRole
   source_profile = thetawavedev
   role_session_name = <A_NAME_FOR_YOUR_SESSION>
   ```
   On Windows, the aws cli initially looks for these files at `C:\Users\USERNAME\.aws\config` and
   `C:\Users\USERNAME\.aws\credentials`.

   Once the credentials are stored locally, run `./asset_manager.py download free_assets` or `./asset_manger.py
   --profile <YOUR_AWS_PROFILE_NAME> download premium_assets` to invoke the AWS CLI. Or look at that script to figure
   out how to invoke the AWS CLI yourself.

   If you instead want to store these credentials outside of the standard `~/.aws` directory, for example in a relative
   `.aws` directory, you may prefix commands with environment variables like
   ```bash
   AWS_SHARED_CREDENTIALS_FILE=path/to/credentials AWS_CONFIG_FILE=path/to/config \
        ./asset_manager.py --profile YOUR_PROFILE_NAME download
   ```

   AWS has more docs about [configuring
   credentials](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html). The aws-cli also has
   documentation in `aws help config-vars`.
0. *(Optional)* If you are using another open source tool for interacting with S3, such as
   [`rclone`](https://github.com/rclone/rclone), [MinIO client](https://github.com/minio/mc), you may need to use
   `--s3-profile` or do something like the following.
```bash
export AWS_SESSION_TOKEN=$(aws sts assume-role --profile thetawavedev --output text \
    --role-arn arn:aws:iam::656454124102:role/ThetawaveDeveloperRole \
    --role-session-name vpsession --query="Credentials.SessionToken" )
```

## Extra Nuglets

To delete just the premium assets from your local assets directory, run the following from the repo root.

```bash
aws --profile <YOUR_AWS_PROFILE_NAME> s3 ls --recursive s3://assets-thetawave/premium_assets/ \
    | awk '{print $4}' | sed 's/^premium_//' \ # Map into our "overlayed free-premium directory structure"
    | xargs rm -f
```
