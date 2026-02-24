# MathHook: High-performance educational computer algebra system
# Re-export everything from the Rust native extension
from .mathhook import *

__all__ = ["mathhook"]
