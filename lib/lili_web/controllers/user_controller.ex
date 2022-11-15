# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
defmodule LiliWeb.UserController do
  use LiliWeb, :controller

  alias Lili.Account
  alias Lili.Account.User
  alias LiliWeb.UserAuth

  def create(conn, params)
  def create(conn, %{"user" => user_params}) do
    case Account.register_user(user_params) do
      {:ok, %User{} = user} ->
        conn
        |> put_status(:created)
        |> put_resp_header("location", Routes.user_path(conn, :show, user))
        |> UserAuth.login_user(user)
      {:error, %Ecto.Changeset{} = changeset} ->
        conn
        |> put_status(:bad_request)
        |> render("400.json", changeset: changeset)
    end
  end
  
  def show(conn, params)
  def show(conn, %{"id" => id}) do
    user = Account.get_user!(id)
    render(conn, "show.json", user: user)
  end
end
