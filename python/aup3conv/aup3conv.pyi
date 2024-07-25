from _aup3conv import Project, Label


__all__ = ["open", "get_labels"]


def open(path: str) -> Project:
    """Open Audacity project file."""
    ...

def get_labels(path: str) -> list[Label]:
    """Retrieve list of labels in project file."""
    ...
