defmodule LiliWeb.UserAuth do
  import Plug.Conn
  import Phoenix.Controller

  alias Lili.Account
  alias LiliWeb.Router.Helpers, as: Routes

  @doc """
  Logs a user in

  """
  def login_user(conn, user, params \\ %{}) do
    token = Account.generate_user_session_token(user)
    conn
    |> put_status(:ok)
    |> render("token.json", token: token)
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
