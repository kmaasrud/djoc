import setuptools

with open("README.md", "r", encoding="utf-8") as f:
    long_description = f.read()

setuptools.setup(
    name="kodb", # Replace with your own username
    version="0.0.2",
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
        "requests"
    ],
    entry_points={
        "console_scripts": [
            "kodb = kodb:main"
        ]
    }
)