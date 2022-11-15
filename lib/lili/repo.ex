# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
defmodule Lili.Repo do
  use Ecto.Repo,
    otp_app: :lili,
    adapter: Ecto.Adapters.Postgres
end
