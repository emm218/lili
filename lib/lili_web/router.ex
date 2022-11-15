defmodule LiliWeb.Router do
  use LiliWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/api", LiliWeb do
    pipe_through :api

    post "/users/register", UserController, :create
    post "/users/login", UserSessionController, :create
    get "/users/:id", UserController, :show
  end

  # Enables the Swoosh mailbox preview in development.
  #
  # Note that preview only shows emails that were sent by the same
  # node running the Phoenix server.
  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through [:fetch_session, :protect_from_forgery]

      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end
