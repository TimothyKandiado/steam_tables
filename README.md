## Steam Tables

# Description

This software is a Chemical Engineering Utility for easily lookup in steam tables and
Water properties table.

The software automatically interpolates depending on the inputs given, 

# saturated steam table

This software contains two saturated steam tables, one that depends on Temperature and the other that depends on Pressure.
These tables use Linear interpolation since it only depends on one Variable

# water properties table

The water properties table is for looking up properties of compressed liquid water, superheated steam and supercritical fluid

This table uses Double Linear Interpolation because the properties depend on both Temperature and Pressure.

The Intepolation is highly inaccurate if it is occuring between different phases as such it returns the closest properties to the provided values of Temperature and Pressure avoiding interpolating them.