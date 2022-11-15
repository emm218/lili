defmodule LiliWeb.UserView do
  use LiliWeb, :view
  alias LiliWeb.UserView

  def render("index.json", %{user: users}) do
    %{data: render_many(users, UserView, "user.json")}
  end

  def render("show.json", %{user: user}) do
    %{data: render_one(user, UserView, "user.json")}
  end

  def render("user.json", %{user: user}) do
    %{
      id: user.id,
      username: user.username,
      email: user.email
    }
  end
  
  def render("400.json", %{changeset: changeset}) do
    Ecto.Changeset.traverse_errors(changeset, &translate_error/1) 
  end
end
