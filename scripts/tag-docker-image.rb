#!/usr/bin/env ruby

require 'json'
require 'octokit'

puts "searching tag in event payload"
event_file=ENV['GITHUB_EVENT_PATH']
token=ENV["GITHUB_TOKEN"]
puts event_file
payload=JSON.parse(File.read(event_file), symbolize_names: true)


if payload[:ref_type] != "tag"
  puts "Event is of type #{payload[:ref_type]}. We are done here."
  exit 0
end

puts "event is of type tag. Awesome :)"

# use env var instead? It is prefixed with refs/tags/foo
#ref=ENV["GITHUB_REF"]
puts "env ref is #{ENV['GITHUB_REF']}"
puts "env sha is #{ENV['GITHUB_SHA']}"

ref=payload[:ref]

# consider using env var instead
#ref=ENV["GITHUB_REPOSITORY"]
repo=payload.fetch(:repository).fetch(:full_name)

pp payload


client =Octokit::Client.new(:access_token => token)
response = client.ref(repo, "tags/#{ref}")

pp response

sha=response.object.sha

puts "we can tag docker image (sha #{sha}) tag new tag '#{ref}'"
image_name="plus9k-rust"

puts "Going to run: 'docker tag #{image_name}:hash-#{sha} docker.pkg.github.com/#{repo}/#{image_name}:#{ref}'"
`docker tag #{image_name}:hash-#{sha} docker.pkg.github.com/#{repo}/#{image_name}:#{ref}`
