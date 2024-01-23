# coding: utf-8

"""
    Agent Protocol

    Specification of the API protocol for communication with an agent.

    The version of the OpenAPI document: v1
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest
import datetime

from agent_protocol_client.models.get_agent_task404_response import (
    GetAgentTask404Response,
)


class TestGetAgentTask404Response(unittest.TestCase):
    """GetAgentTask404Response unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> GetAgentTask404Response:
        """Test GetAgentTask404Response
        include_option is a boolean, when False only required
        params are included, when True both required and
        optional params are included"""
        # uncomment below to create an instance of `GetAgentTask404Response`
        """
        model = GetAgentTask404Response()
        if include_optional:
            return GetAgentTask404Response(
                message = 'Unable to find entity with the provided id'
            )
        else:
            return GetAgentTask404Response(
                message = 'Unable to find entity with the provided id',
        )
        """

    def testGetAgentTask404Response(self):
        """Test GetAgentTask404Response"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)


if __name__ == "__main__":
    unittest.main()
