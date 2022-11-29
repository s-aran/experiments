from rest_framework.views import APIView
from rest_framework.response import Response
from rest_framework.permissions import IsAuthenticated

# Create your views here.


class SayHello(APIView):
    permission_classes = ()

    def post(self, request):
        return Response("Hello!")


class SecretSayHello(APIView):
    permission_classes = (IsAuthenticated,)

    def post(self, request) -> Response:
        return Response("Secret Hello!!")
