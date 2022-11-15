# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
defmodule LiliWeb.UserSessionController do
  use LiliWeb, :controller

  alias Lili.Account
  alias LiliWeb.UserAuth

  def create(conn, params)
  def create(conn, %{"user" => user_params}) do
    %{"username" => username, "password" => password} = user_params

    if user = Account.get_user_by_username_and_password(username, password) do
      UserAuth.login_user(conn, user)
    else
      conn
      |> put_status(:unauthorized)
      |> render(LiliWeb.ErrorView, "401.json")
    end
  end

  def delete(conn, _params) do
    conn
    |> UserAuth.logout_user()
  end
end
