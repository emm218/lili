defmodule LiliWeb.UserController do
  use LiliWeb, :controller

  alias Lili.Account
  alias Lili.Account.User
  
  def create(conn, params)
  def create(conn, %{"user" => user_params}) do
    with {:ok, %User{} = user} <- Account.register_user(user_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", Routes.user_path(conn, :show, user))
      |> render("show.json", user: user)
    end
  end
  
  def show(conn, params)
  def show(conn, %{"id" => id}) do
    user = Account.get_user!(id)
    render(conn, "show.json", user: user)
  end
end
