# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
defmodule Lili.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Start the Ecto repository
      Lili.Repo,
      # Start the Telemetry supervisor
      LiliWeb.Telemetry,
      # Start the PubSub system
      {Phoenix.PubSub, name: Lili.PubSub},
      # Start the Endpoint (http/https)
      LiliWeb.Endpoint
      # Start a worker by calling: Lili.Worker.start_link(arg)
      # {Lili.Worker, arg}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Lili.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    LiliWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
