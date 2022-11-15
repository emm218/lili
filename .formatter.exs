# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
[
  import_deps: [:ecto, :phoenix],
  inputs: ["*.{ex,exs}", "priv/*/seeds.exs", "{config,lib,test}/**/*.{ex,exs}"],
  subdirectories: ["priv/*/migrations"]
]
