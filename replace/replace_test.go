package replace

import "testing"

func Test_searchAndReplaceInBetweenTokens(t *testing.T) {
	type args struct {
		leftToken  string
		rightToken string
		s          string
		old        string
		new        string
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "",
			args: args{
				leftToken:  "[S]",
				rightToken: "[X]",
				s:          "abc abc [S] def def [X] def def [S]jkl jkl [X] mno mno",
				old:        "def",
				new:        "XYZ",
			},
			want: "abc abc [S] XYZ XYZ [X] def def [S]jkl jkl [X] mno mno",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := SearchAndReplaceInBetweenTokens(tt.args.leftToken, tt.args.rightToken, tt.args.s, tt.args.old, tt.args.new); got != tt.want {
				t.Errorf("\n got: %v, \nwant: %v", got, tt.want)
			}
		})
	}
}
