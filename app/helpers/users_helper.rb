module UsersHelper

  def gravatar_for(user)
    gravatar_id = Digest::MD5::hexdigest(user.account_id)
    gravatar_url = "https://secure.gravatar.com/avatar/#{gravatar_id}?d=identicon&r=PG"
    image_tag(gravatar_url, class: "gravatar")
  end
end
