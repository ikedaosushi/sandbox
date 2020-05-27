from djoser.serializers import TokenSerializer
from rest_framework import serializers
from stream_chat import StreamChat
from djoser.conf import settings as djoser_settings
from django.conf import settings


print(djoser_settings)
print(settings)

class StreamTokenSerializer(TokenSerializer):
    stream_token = serializers.SerializerMethodField()

    class Meta:
        model = djoser_settings.TOKEN_MODEL
        fields = ('auth_token', 'stream_token')

    def get_stream_token(self, obj):
        client = StreamChat(api_key=settings.STREAM_API_KEY, api_secret=settings.STREAM_API_SECRET)
        token = client.create_token(obj.user.id)

        return token