from pathlib import Path

import pytest

import aup3conv as ac


@pytest.fixture
def path():
    return "data/test-project.aup3"

@pytest.fixture
def project(path):
    return ac.open(path)


def test_get_labels(path) -> None:
    labels = ac.get_labels(path)
    assert isinstance(labels, list)
    for item in labels:
        assert hasattr(item, "title")
        assert hasattr(item, "start")
        assert hasattr(item, "stop")

def test_open(project) -> None:
    assert hasattr(project, "path")
    assert hasattr(project, "labels")
    assert hasattr(project, "fps")

def test_project_load_waveblock(project) -> None:
    block = project.load_waveblock(1)
    assert isinstance(block, list)
    assert all(isinstance(item, float) for item in block)


def test_label_str(path) -> None:
    labels = ac.get_labels(path)
    assert labels[0].__str__() == "Label(title='1', start=26.79582766439909, stop=28.978503401360545)"

def test_label_repr(path) -> None:
    labels = ac.get_labels(path)
    assert labels[0].__repr__() == "Label(title='1', start=26.79582766439909, stop=28.978503401360545)"


