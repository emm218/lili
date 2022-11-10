defmodule Lili.Repo do
  use Ecto.Repo,
    otp_app: :lili,
    adapter: Ecto.Adapters.Postgres
end
