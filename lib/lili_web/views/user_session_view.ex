defmodule LiliWeb.UserSessionView do
  use LiliWeb, :view

  def render("token.json", %{token: token}) do
    %{
      token: token
    }
  end
  
end
