s.boot;

s.waitForBoot {

	SynthDef(\tone, {|freq = 440|
		var sig = SinOsc.ar(freq);
		Out.ar(0, [sig, sig]);
	}).add;

	SynthDef(\trem, {|depth = 0.5, speed = 2|
		var in = SinOsc.ar(440);
		var trem = SinOsc.ar(speed, mul: depth);
		var mod = (1 - depth) + trem;
		var sig = in * mod;
		Out.ar(0, [sig, sig]);
	}).add;

	~synth = nil;

	OSCresponderNode(nil, "/start", {|t, r, msg|
		msg.postln;
		~synth = Synth.new(msg[1].asSymbol);
	}).add;

	OSCresponderNode(nil, "/stop", {|t, r, msg|
		msg.postln;
		~synth.free;
	}).add;

	OSCresponderNode(nil, "/set", {|t, r, msg|
		msg.postln;
		~synth.set(msg[1].asSymbol, msg[2]);
	}).add;
}