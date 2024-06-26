#!/usr/bin/env python

"""This program shows parametrized `hyperfine` benchmark results as an
errorbar plot."""

import argparse
import json
import matplotlib.pyplot as plt
import sys
import numpy as np
import seaborn as sns

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("file", help="JSON file with benchmark results", nargs="+")
parser.add_argument(
    "--parameter-name",
    metavar="name",
    type=str,
    help="Deprecated; parameter names are now inferred from benchmark files",
)
parser.add_argument(
    "--log-x", help="Use a logarithmic x (parameter) axis", action="store_true"
)
parser.add_argument(
    "--log-time", help="Use a logarithmic time axis", action="store_true"
)
parser.add_argument(
    "--titles", help="Comma-separated list of titles for the plot legend"
)
parser.add_argument(
    "-o", "--output", help="Save image to the given filename."
)

args = parser.parse_args()
if args.parameter_name is not None:
    sys.stderr.write(
        "warning: --parameter-name is deprecated; names are inferred from "
        "benchmark results\n"
    )


def die(msg):
    sys.stderr.write("fatal: %s\n" % (msg,))
    sys.exit(1)


def extract_parameters(results):
    """Return `(parameter_name: str, parameter_values: List[float])`."""
    if not results:
        die("no benchmark data to plot")
    (names, values) = zip(*(unique_parameter(b) for b in results))
    names = frozenset(names)
    if len(names) != 1:
        die(
            "benchmarks must all have the same parameter name, but found: %s"
            % sorted(names)
        )
    return next(iter(names)), list(values)


def unique_parameter(benchmark):
    """Return the unique parameter `(name: str, value: float)`, or die."""
    params_dict = benchmark.get("parameters", {})
    if not params_dict:
        die("benchmarks must have exactly one parameter, but found none")
    if len(params_dict) > 1:
        die(
            "benchmarks must have exactly one parameter, but found multiple: %s"
            % sorted(params_dict)
        )
    [(name, value)] = params_dict.items()
    return name, float(value)


plt.rcParams.update({'font.size': 16})
fig_size = (13.66, 7.2)

plt.figure(figsize=fig_size, constrained_layout=True)
sns.set_style("darkgrid")

parameter_name = None

for filename in args.file:
    with open(filename) as f:
        results = json.load(f)["results"]

    (this_parameter_name, parameter_values) = extract_parameters(results)
    if parameter_name is not None and this_parameter_name != parameter_name:
        die(
            "files must all have the same parameter name, but found %r vs. %r"
            % (parameter_name, this_parameter_name)
        )
    if parameter_name is None:
        parameter_name = this_parameter_name
        plt.xticks(parameter_values)
        seq = np.arange(1, 8)
        plt.plot(seq, seq)

    times_mean = [b["mean"] for b in results]
    seq_time = times_mean[0]
    times_mean = [seq_time / b for b in times_mean]

    plt.plot(parameter_values, times_mean)

plt.xlabel(parameter_name.title().replace("_", " "))
plt.ylabel("Speedup")

if args.log_time:
    plt.yscale("log")

if args.log_x:
    plt.xscale("log")

if args.titles:
    plt.legend(["ideal"] + args.titles.split(","))

if args.output:
    plt.savefig(args.output)
else:
    plt.show()
