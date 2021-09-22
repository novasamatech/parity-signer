#!/bin/bash
# This is temporary solution

find ./src -name '*.ts?' -exec sed -i -e 's/Copyright 2015-2020 Parity Technologies (UK) Ltd./Copyright 2015-2021 Parity Technologies (UK) Ltd./g' {} \;
find ./src -name '*.ts' -exec sed -i -e 's/Copyright 2015-2020 Parity Technologies (UK) Ltd./Copyright 2015-2021 Parity Technologies (UK) Ltd./g' {} \;
find ./ios -name '*.h' -exec sed -i -e 's/Copyright 2015-2020 Parity Technologies (UK) Ltd./Copyright 2015-2021 Parity Technologies (UK) Ltd./g' {} \;
find ./rust/ -name '*.rs' -exec sed -i -e 's/Copyright 2015-2020 Parity Technologies (UK) Ltd./Copyright 2015-2021 Parity Technologies (UK) Ltd./g' {} \;
