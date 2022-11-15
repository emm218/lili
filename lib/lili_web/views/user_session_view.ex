# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
defmodule LiliWeb.UserSessionView do
  use LiliWeb, :view

  def render("token.json", %{token: token}) do
    %{
      token: token
    }
  end
  
end
