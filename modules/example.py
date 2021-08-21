#!/bin/python

import sys
import json


def pre_after_script(reqid, jobname):
    """Before after script is executed.

    Args:
        reqid: id of the request
        jobname: name of the job
    """
    pass


def post_after_script(reqid, jobname, command_status):
    """Post after script.

    Args:
        reqid: id of the request
        jobname: name of the job
        command_status: status of the command
    """
    pass


def pre_before_script(reqid, jobname):
    """Before before script is executed.

    Args:
        reqid: id of the request
        jobname: name of the job
    """
    pass


def post_before_script(reqid, jobname, command_status):
    """Post before script.

    Args:
        reqid: id of the request
        jobname: name of the job
        command_status: status of the command
    """
    pass


def pre_main_script(reqid, jobname):
    """Before main script.

    Args:
        reqid: id of the request
        jobname: name of the job
    """
    pass


def post_main_script(reqid, jobname, command_status):
    """After main script is executed.

    Args:
        reqid: id of the request
        jobname: name of the job
        command_status: status of the command
    """
    pass


def job_starting(reqid, jobname):
    """When job is starting.

    Args:
        reqid: id of the request
        jobname: name of the job
    """
    pass


def job_finished(reqid, jobname):
    """When job is has finished.

    Args:
        reqid: id of the request
        jobname: name of the job
    """
    pass


def request_starting(reqid):
    """When request has started

    Args:
        reqid: id of the request
    """
    pass


def request_finished(reqid, status):
    """When has finished is has finished.

    Args:
        reqid: id of the request
        status: status of the request
    """
    pass


if __name__ == '__main__':
    opts = [opt for opt in sys.argv[1:] if opt.startswith("-")]
    args = [arg for arg in sys.argv[1:] if not arg.startswith("-")]

    payload = {}
    for a in args:
        print(a)
        payload[a.split(":")[0].replace("\"", "")] = a.split(":")[
            1].replace("\"", "")

    if "-rf" in opts:
        request_finished(**payload)
    elif "-rs" in opts:
        request_starting(**json.loads(args[0]))
    elif "-jf" in opts:
        job_finished(**payload)
    elif "-js" in opts:
        job_starting(**payload)
    elif "-post_ms" in opts:
        post_main_script(**payload)
    elif "-pre_ms" in opts:
        pre_main_script(**payload)
    elif "-post_bs" in opts:
        post_before_script(**payload)
    elif "-pre_bs" in opts:
        pre_before_script(**payload)
    elif "-post_as" in opts:
        post_after_script(**payload)
    elif "-pre_as" in opts:
        pre_after_script(**payload)
