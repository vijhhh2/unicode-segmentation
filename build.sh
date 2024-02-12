#!/bin/bash

# Check if at least one argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: You need to provide version number. Ex: $0 0.1.0"
    exit 1
fi

echo "Vestion number is: $1"

file_path="dist/package.json"
readme_file_path="readme.md"
readme_file_dest_path="dist/readme.md"
package_json="{
    \"name\": \"unicode_segmentation\",
    \"version\": \"$1\",
    \"description\": \"This library provides node binding for rust package Unicode Segmentation Crate\",
    \"author\": {
        \"name\": \"Vijay\",
        \"email\": \"vijay.mangapoti@gmail.com\",
        \"url\": \"https://github.com/vijhhh2\"
    },
    \"repository\": {
        \"url\": \"git+https://github.com/vijhhh2/unicode-segmentation.git\"
    },
    \"keywords\": [
        \"unicode\",
        \"unicode_segmentation\",
        \"grapheme\"
    ],
    \"files\": [
        \"index.node\",
        \"lib.js\",
        \"lib.d.ts\"
    ],
    \"module\": \"lib.js\",
    \"main\": \"lib.js\",
    \"types\": \"lib.d.ts\",
    \"license\": \"MIT\"
}"

# Create a release build
export TSLINK_BUILD=true
nj-cli build --release

cat <<EOF > "$file_path"
$package_json
EOF

echo "written $file_path"

# Copy the readme file to dist
cp "$readme_file_path" "$readme_file_dest_path"

# publish the package to npm
cd ./dist || exit
npm publish --access public
