# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: example.proto
# plugin: python-betterproto
from dataclasses import dataclass

import betterproto


@dataclass
class Greeting(betterproto.Message):
    """Greeting represents a message you can tell a user."""

    field1: str = betterproto.string_field(1)
    field2: str = betterproto.string_field(2)
    # string field3 = 3;
    field4: str = betterproto.string_field(4)