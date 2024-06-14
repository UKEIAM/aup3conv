import pytest

import aup3conv as ac


@pytest.fixture
def path():
    return "data/test-project.aup3"

@pytest.fixture
def project(path):
    return ac.open(path)


def test_open(project) -> None:
    assert hasattr(project, "fps")
    assert hasattr(project, "labels")
    assert hasattr(project, "load_audio")
    assert hasattr(project, "path")

def test_fps(project) -> None:
    assert isinstance(project.fps, int)

def test_labels(project) -> None:
    assert isinstance(project.labels, list)
    for item in project.labels:
        assert hasattr(item, "title")
        assert hasattr(item, "start")
        assert hasattr(item, "stop")

def test_load_audio(project) -> None:
    snd = project.load_audio()
    assert isinstance(snd, list)
