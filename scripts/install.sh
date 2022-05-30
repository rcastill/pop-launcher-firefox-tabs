#!/bin/bash

if ! command -v curl &> /dev/null
then
    read -p "curl not found: use apt to install? [y/N] " install_curl
    if [ "$install_curl" != "y" ]; then exit 1; fi
    sudo apt-get -y install curl
fi

repo_name=pop-launcher-firefox-tabs
bin_name=$repo_name
repo_user=rcastill

url="https://api.github.com/repos/${repo_user}/${repo_name}/releases/latest"

# request latest release info
response=$(curl -s --fail $url)

if [ $? -ne 0 ]
then
    issues_url="https://github.com/${repo_user}/${repo_name}/issues"
    >&2 echo "Latest release not found. Please report an issue at ${issues_url}."
    exit 1
fi

# get url to download bin and .ron file
# *a little bit unstable but jq is not needed*
base_url=$(echo $response | sed -e 's/.*browser_download_url" *: *"\([^"]*\)".*/\1/g' | xargs dirname)
bin_url="${base_url}/${bin_name}"
ron_url="${base_url}/plugin.ron"

# make tmp download dir
tmp_dir="${repo_name}.d.tmp"
mkdir -p $tmp_dir

cleanup() {
    rm -rf $tmp_dir
}

trap cleanup EXIT ERR

bail() {
    >&2 echo "$@"
    exit 1
}

# download assets
>&2 echo "INFO: Downloading from ${base_url}"
curl --fail -Lo "${tmp_dir}/firefox-tabs" "$bin_url" || bail "ERROR: Failed downloading plugin binary"
curl --fail -Lo "${tmp_dir}/plugin.ron" "$ron_url" || bail "ERROR: Failed downloading plugin.ron"
chmod u+x "${tmp_dir}/firefox-tabs"

# install
install_dir="$HOME/.local/share/pop-launcher/plugins/firefox-tabs"
mkdir -p $install_dir
mv -v ${tmp_dir}/* $install_dir
>&2 echo "INFO: Done"