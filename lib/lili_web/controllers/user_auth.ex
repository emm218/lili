# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
defmodule LiliWeb.UserAuth do
  import Plug.Conn
  import Phoenix.Controller

  alias Lili.Account
  alias LiliWeb.Router.Helpers, as: Routes

  @doc """
  Logs a user in

  """
  def login_user(conn, user) do
    token = Account.generate_user_session_token(user)
    conn
    |> put_status(:ok)
    |> render(LiliWeb.UserSessionView, "token.json", token: token)
  end

  @doc """
  Logs the user out
  """
  def logout_user(conn)
  def logout_user(conn) do
    with user_token = Plug.Conn.get_req_header(conn, "authorization") do
      Account.delete_user_session_token(user_token)
    end

    conn
    |> put_status(:ok)
    |> render("200.json")
  end
end
