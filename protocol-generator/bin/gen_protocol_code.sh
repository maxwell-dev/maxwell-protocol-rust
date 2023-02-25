#!/bin/bash

current_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )";
cd ${current_dir}

# Fetch proto files
if [[ ! -d proto ]]; then
    git clone -b dev https://github.com/maxwell-dev/maxwell-protocol.git proto;
fi

output_dir="../src/protocol"
mkdir -p ${output_dir}

touch ${output_dir}/mod.rs
echo "
mod maxwell_protocol;
mod maxwell_protocol_ext;

pub use self::maxwell_protocol::*;
pub use self::maxwell_protocol_ext::*;
" > ${output_dir}/mod.rs

bin/normalize_type_name.py --proto_file proto/maxwell_protocol.proto

# Generate xxx file by protoc
cargo run && mv ${output_dir}/maxwell.protocol.rs ${output_dir}/maxwell_protocol.rs

# Generate xxx_ext file
bin/gen_protocol_ext.py \
    --proto_file proto/maxwell_protocol.normalized.proto \
    --enum_type_name MsgType
