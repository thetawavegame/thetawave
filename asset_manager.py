#!/usr/bin/env python3
"""Download and upload Thetawave assets to our S3 bucket. This CLI basically wraps the `aws s3 sync` command. See the
CONTRIBUTING.md for more information on executing this script, including properly setting up credentials."""
from __future__ import annotations
from argparse import (
    ArgumentParser,
    Namespace,
    BooleanOptionalAction,
    ArgumentDefaultsHelpFormatter,
)
from collections.abc import Sequence
from enum import Enum
from subprocess import check_call
from sys import stderr, stdout
import logging

LOGGER = logging.getLogger(__name__)


class S3Action(str, Enum):
    """The operation to do on the thetawave S3 bucket"""

    DOWNLOAD = "download"
    UPLOAD = "upload"


class S3Location(str, Enum):
    """Whether to (down|up)load the free or premium assets. These are separate prefixes in the same S3 bucket"""

    FREE = "free_assets"
    PREMIUM = "premium_assets"


class TWAssetCLIArgs(Namespace):
    command: S3Action
    profile: str | None
    s3_location: S3Location
    dryrun: bool


def get_parser() -> ArgumentParser:
    res = ArgumentParser(description=__doc__, formatter_class=ArgumentDefaultsHelpFormatter)
    res.add_argument(
        "command",
        choices=[x.value for x in S3Action.__members__.values()],
        help=S3Action.__doc__,
        type=S3Action,
    )
    res.add_argument(
        "--profile",
        help="""The name of the AWS profile that can access a role for manipulating the thetawave bucket. This profile
        must be permissioned to use the AWS IAM role arn:aws:iam::656454124102:role/ThetawaveDeveloperRole . Your
        default profile _probably_ doesnt/shouldn't correspond to your thetawave developer IAM user account, so you
        probably need to set this. If this is nor provided, AWS specified waterfall logic determines the credentials
        used to access the S3 bucket. https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html""",
        required=False,
    )
    res.add_argument(
        "--dryrun",
        help="""If provided, prints out all of the copied files WITHOUT copying them to/from S3. Useful for
        testing/looking before one leaps.""",
        action=BooleanOptionalAction,
        dest="dryrun",
        default=True,
    )
    res.add_argument(
        "--s3-location",
        help=S3Location.__doc__,
        choices=[x.value for x in S3Location.__members__.values()],
        type=S3Location,
        default=S3Location.FREE.value,
    )
    return res


def run(args: TWAssetCLIArgs) -> None:
    LOGGER.info(f"Running asset operation with {args=}")
    if (args.s3_location, args.command) == (S3Location.PREMIUM, S3Action.UPLOAD):
        raise ValueError("For now we will upload premium assets by directly calling the AWS CLI.")
    s3_loc = f"s3://assets-thetawave/{args.s3_location.value}/"
    src_dest = [s3_loc, "assets/"] if args.command == S3Action.DOWNLOAD else ["assets/", s3_loc]
    action_args = [
        "aws",
        *(["--profile", args.profile] if args.profile else []),
        "s3",
        "sync",
        *("--exclude", "*.ron", "--exclude", "*.gif"),
        *(["--dryrun"] if args.dryrun else []),
        *src_dest,
    ]
    LOGGER.info(f"Running command with args: {' '.join(action_args)}")
    check_call(action_args, stdout=stdout, stderr=stderr)
    if args.dryrun:
        LOGGER.warning(
            """The asset manager was executed with the --dryrun flag. No action was taken. Rerun with
           --no-dryrun if the desired files will be (down|up)loaded."""
        )


def main(args: Sequence[str] | None = None) -> None:
    parser = get_parser()
    args_: TWAssetCLIArgs = parser.parse_args(args, TWAssetCLIArgs())  # type: ignore
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s %(levelname)s %(module)s %(funcName)s %(message)s",
        stream=stdout,
    )
    run(args_)


if __name__ == "__main__":
    main()
