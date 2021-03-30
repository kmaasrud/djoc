import setuptools

with open("README.md", "r", encoding="utf-8") as f:
    long_description = f.read()

setuptools.setup(
    name="kodb",
    version="0.1.6",
    author="kmaasrud",
    author_email="km@aasrud.com",
    description="kmaasrud's opinionated document builder",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/kmaasrud/kodb",
    packages=setuptools.find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires='>=3.6',
    install_requires=[
        "pandoc-xnos",
        "pandoc-fignos",
        "pandoc-eqnos",
        "pandoc-tablenos",
        "pandoc-secnos"
    ],
    entry_points={
        "console_scripts": [
            "kodb = kodb:main"
        ]
    }
)
