from collections import namedtuple

class SemVer(namedtuple("SemVer", ["major", "minor", "patch", "pre"])):
    @classmethod
    def parse(cls, s):
        s = "".join(s.split())
        halves = s.split("-")
        assert 1 <= len(halves) <= 2
        parts = halves[0]
        pre = None if len(halves) < 2 else halves[1]
        major, minor, patch, *rest = parts.split(".")
        assert not rest
        return cls(int(major), int(minor), int(patch), pre)

    def __str__(self):
        s = f"{self.major}.{self.minor}.{self.patch}"
        if self.pre:
            s = f"{s}-{self.pre}"
        return s
